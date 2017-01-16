#!/usr/bin/env ruby

$: << "#{__dir__}/lib"

require "bridge/output"

module Bridge
  class SIMD
    TYPES = {
      char: { size: 1, max_width: 16, type: 'i8' },
      uchar: { size: 1, max_width: 16, type: 'u8' },
      short: { size: 2, max_width: 16, type: 'i16' },
      ushort: { size: 2, max_width: 16, type: 'u16' },
      int: { size: 4, max_width: 16, type: 'i32' },
      uint: { size: 4, max_width: 16, type: 'u32' },
      float: { size: 4, max_width: 16, type: 'f32', max_matrix_size: 4 },
      long: { size: 8, max_width: 8, type: 'i64' },
      ulong: { size: 8, max_width: 8, type: 'u64' },
      double: { size: 8, max_width: 8, type: 'f64', max_matrix_size: 4 }
    }

    WIDTHS = [1, 2, 3, 4, 8, 16]

    def self.generate(o)
      o.puts("#![allow(non_camel_case_types)]")
      TYPES.each do |type, attributes|
        WIDTHS.select { |x| x < attributes.fetch(:max_width) }.each do |width|
          name = "vector_#{type}#{width}"

          if width == 1
            o.puts("pub type #{name} = #{attributes.fetch(:type)};")
          else
            content = ([attributes.fetch(:type)] * width).join(", ")

            o.puts
            o.puts("#[repr(simd)]")
            o.puts("#[derive(Copy, Clone, Debug)]")
            o.puts("pub struct #{name}(#{content});")
          end
        end
      end

      TYPES.select { |_, attributes| attributes[:max_matrix_size] }.each do |type, attributes|
        max_matrix_size = attributes.fetch(:max_matrix_size)
        
        (2 .. max_matrix_size).each do |i|
          vector_name = "vector_#{type}#{i}"

          (2 .. max_matrix_size).each do |j|
            name = "matrix_#{type}#{j}x#{i}"
            content = ([vector_name] * j).join(", ")

            o.puts
            o.puts("#[repr(C)]")
            o.puts("#[derive(Copy, Clone, Debug)]")
            o.puts("pub struct #{name}(#{content});")
          end
        end
      end
    end
  end
end

if $0 == __FILE__
  Bridge::SIMD.generate(Bridge::Output.new(STDOUT))
end
