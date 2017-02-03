#!/usr/bin/env ruby

$: << "#{__dir__}/lib"

require "bridge/output"

module Bridge
  class SIMD
    TYPES = [
      # { size: 1, max_width: 4, type: 'i8', opencl: 'char', signed: true },
      # { size: 1, max_width: 4, type: 'u8', opencl: 'uchar', unsigned: true },
      # { size: 2, max_width: 4, type: 'i16', opencl: "short", signed: true },
      # { size: 2, max_width: 4, type: 'u16', opencl: "ushort", unsigned: true },
      { size: 4, max_width: 4, type: 'i32', opencl: "int", kind: %i(signed integer) },
      # { size: 4, max_width: 4, type: 'u32', opencl: "uint", unsigned: true },
      { size: 4, max_width: 4, type: 'f32', opencl: "float", kind: %i(float), max_matrix_size: 4 },
      # { size: 8, max_width: 4, type: 'i64', opencl: "long", signed: true },
      # { size: 8, max_width: 4, type: 'u64', opencl: "ulong", unsigned: true },
      # { size: 8, max_width: 4, type: 'f64', opencl: "double", float: true, max_matrix_size: 4 }
    ]

    WIDTHS = [1, 2, 3, 4, 8, 16]

    def self.generate(path)
      FileUtils.mkdir_p(path)

      files = []

      files << generate_mod(path)

      TYPES.each do |attributes|
        scalar = attributes.fetch(:type)
        type = attributes.fetch(:opencl)
        kind = attributes.fetch(:kind)

        WIDTHS.select { |x| x <= attributes.fetch(:max_width) }.each do |width|
          io = StringIO.new
          o = Bridge::Output.new(io)

          name = "#{type}#{width}"

          if width == 1
            o.puts("pub type #{name} = #{attributes.fetch(:type)};")
            o.puts("pub type vector_#{name} = #{name};")
          else
            content = (["pub #{attributes.fetch(:type)}"] * width).join(", ")

            o.puts("use std;")
            o.puts("use simd::Dot;")
            o.puts("use simd::types::*;")
            o.puts
            o.puts("#[repr(simd)]")
            o.puts("#[derive(Copy, Clone, Debug)]")
            o.puts("pub struct #{name}(#{content});")
            o.puts("pub type vector_#{name} = #{name};")
            o.puts
            o.block("extern \"platform-intrinsic\"") do
              o.puts("fn simd_add<T>(x: T, y: T) -> T;")
              o.puts("fn simd_sub<T>(x: T, y: T) -> T;")
              o.puts("fn simd_mul<T>(x: T, y: T) -> T;")
              o.puts("fn simd_div<T>(x: T, y: T) -> T;")
            end
            %w(add sub mul div).each do |op|
              o.puts
              o.block("impl std::ops::#{op.capitalize} for #{name}") do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: Self) -> Self") do |o|
                  o.puts("return unsafe { simd_#{op}(self, other) };")
                end
              end
              o.puts
              o.block("impl std::ops::#{op.capitalize}<#{scalar}> for #{name}") do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: #{scalar}) -> Self") do |o|
                  o.puts("return unsafe { simd_#{op}(self, #{name}::broadcast(other)) };")
                end
              end
              o.puts
              o.block("impl std::ops::#{op.capitalize}<#{name}> for #{scalar}") do |o|
                o.puts("type Output = #{name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: #{name}) -> #{name}") do |o|
                  o.puts("return unsafe { simd_#{op}(#{name}::broadcast(self), other) };")
                end
              end
            end
            o.puts
            o.block("impl Dot for #{name}") do |o|
              o.puts("type Output = #{scalar};")
              o.puts
              o.puts("#[inline]")
              o.block("fn dot(self, other: #{name}) -> #{scalar}") do |o|
                o.puts("return (self * other).reduce_add();")
              end
            end
            o.puts
            o.block("impl #{name}") do |o|
              o.puts("#[inline]", pad: true)
              o.block("pub fn broadcast(x: #{scalar}) -> #{name}") do |o|
                o.puts("return #{name}(#{(["x"] * width).join(", ")});")
              end

              if kind.include?(:float)
                o.puts("#[inline]", pad: true)
                o.block("pub fn length_squared(self) -> #{scalar}") do |o|
                  o.puts("return self.dot(self);")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn normalize(self) -> #{name}") do |o|
                  o.puts("return self / self.length_squared().sqrt();") # TODO: Use some rsqrt approximation
                end
              end

              case width
              when 2
                o.puts("#[inline]", pad: true)
                o.block("pub fn low(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return self.0;")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn high(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return self.1;")
                end
              when 3
                o.puts("#[inline]", pad: true)
                o.block("pub fn low(self) -> #{type}2") do |o|
                  o.puts("return #{type}2(self.0, self.1);")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn high(self) -> #{type}2") do |o|
                  o.puts("return #{type}2(self.2, 0#{".0" if kind.include?(:float)});")
                end
              else
                o.puts("#[inline]", pad: true)
                o.block("pub fn low(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return #{type}#{width / 2}(#{(width / 2).times.map { |i| "self.#{i}"}.join(", ")});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn high(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return #{type}#{width / 2}(#{(width / 2).times.map { |i| "self.#{width / 2 - i - 1}"}.join(", ")});")
                end
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn reduce_add(self) -> #{attributes.fetch(:type)}") do |o|
                case width
                when 2
                  o.puts("return self.0 + self.1;")
                when 3
                  o.puts("return self.0 + self.1 + self.2;")
                else
                  o.puts("return (self.low() + self.high()).reduce_add();")
                end
              end
            end
          end

          files << ["#{path}/type_#{name}.rs", io.string]
        end
      end

      TYPES.select { |attributes| attributes[:max_matrix_size] }.each do |attributes|
        scalar = attributes.fetch(:type)
        type = attributes.fetch(:opencl)
        kind = attributes.fetch(:kind)

        max_matrix_size = attributes.fetch(:max_matrix_size)

        (2 .. max_matrix_size).each do |i|
          vector_name = "#{type}#{i}"

          (2 .. max_matrix_size).each do |j|
            io = StringIO.new
            o = Bridge::Output.new(io)

            name = "#{type}#{j}x#{i}"
            content = (["pub #{vector_name}"] * j).join(", ")

            o.puts("use std;", pad: true)
            o.puts("use simd::Dot;") if i == j
            o.puts("use simd::types::*;")

            o.puts("#[repr(C)]", pad: true)
            o.puts("#[derive(Copy, Clone, Debug)]")
            o.puts("pub struct #{name}(#{content});")
            o.puts("pub type matrix_#{name} = #{name};")

            if i == j
              identity = j.times.map { |k| "#{vector_name}(#{([0.0] * i).tap { |ary| ary[k] = 1.0 }.join(", ")})" }.join(", ")
              o.puts("pub static identity_#{name}: #{name} = #{name}(#{identity});")
              o.puts("pub static matrix_identity_#{name}: #{name} = #{name}(#{identity});")
            end

            %w(add sub mul div).each do |op|
              o.block("impl std::ops::#{op.capitalize} for #{name}", pad: true) do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: Self) -> Self") do |o|
                  o.puts("return #{name}(#{j.times.map { |k| "self.#{k}.#{op}(other.#{k})" }.join(", ")});")
                end
              end

              o.block("impl std::ops::#{op.capitalize}<#{scalar}> for #{name}", pad: true) do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: #{scalar}) -> Self") do |o|
                  o.puts("let other = #{vector_name}::broadcast(other);")
                  o.puts
                  o.puts("return #{name}(#{j.times.map { |k| "self.#{k}.#{op}(other)" }.join(", ")});")
                end
              end

              o.block("impl std::ops::#{op.capitalize}<#{name}> for #{scalar}", pad: true) do |o|
                o.puts("type Output = #{name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: #{name}) -> #{name}") do |o|
                  o.puts("let scalar = #{vector_name}::broadcast(self);")
                  o.puts
                  o.puts("return #{name}(#{j.times.map { |k| "scalar.#{op}(other.#{k})" }.join(", ")});")
                end
              end
            end

            if kind.include?(:float) && i == j
              o.block("impl Dot for #{name}", pad: true) do |o|
                o.puts("type Output = #{name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn dot(self, other: #{name}) -> #{name}") do |o|
                  o.puts("return #{name}(#{j.times.map { |k| "self.dot(other.#{k})" }.join(", ")});")
                end
              end

              o.block("impl Dot<#{vector_name}> for #{name}", pad: true) do |o|
                o.puts("type Output = #{vector_name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn dot(self, other: #{vector_name}) -> #{vector_name}") do |o|
                  o.puts("return #{j.times.map { |k| "self.#{k} * other.#{k}" }.join(" + ")};")
                end
              end
            end

            files << ["#{path}/type_#{name}.rs", io.string]
          end
        end
      end

      files
    end

    def self.generate_mod(path)
      io = StringIO.new
      o = Bridge::Output.new(io)

      TYPES.each do |attributes|
        type = attributes.fetch(:opencl)

        WIDTHS.select { |x| x <= attributes.fetch(:max_width) }.each do |width|
          o.puts("mod type_#{type}#{width};")
        end

        if max_matrix_size = attributes[:max_matrix_size]
          (2 .. max_matrix_size).each do |i|
            (2 .. max_matrix_size).each do |j|
              o.puts("mod type_#{type}#{j}x#{i};")
            end
          end
        end
      end

      o.puts

      TYPES.each do |attributes|
        type = attributes.fetch(:opencl)

        WIDTHS.select { |x| x <= attributes.fetch(:max_width) }.each do |width|
          o.puts("pub use self::type_#{type}#{width}::*;")
        end

        if max_matrix_size = attributes[:max_matrix_size]
          (2 .. max_matrix_size).each do |i|
            (2 .. max_matrix_size).each do |j|
              o.puts("pub use self::type_#{type}#{j}x#{i}::*;")
            end
          end
        end
      end

      ["#{path}/mod.rs", io.string]
    end
  end
end
