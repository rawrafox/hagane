$: << "#{__dir__}/generator"

require "shellwords"

require "objc-bridge-generator"

XML = Rake::FileList["#{__dir__}/generator/xml/*.xml"]
XML_RS = XML.pathmap("#{__dir__}/src/%n/mod.rs")
XML_RS_MAP = XML_RS.zip(XML).to_h

METAL = Rake::FileList["#{__dir__}/**/*.metal"]

task default: [:objc_bridge, :shaders]

task objc_bridge: XML_RS
task shaders: METAL.ext(".metallib")

rule ".metalir" => ".metal" do |t|
  puts "Generating Metal IR (#{t.name} from #{t.source})"
  system("xcrun metal -o #{t.name.shellescape} #{t.source.shellescape}") or exit
end

rule ".metallib" => ".metalir" do |t|
  puts "Generating Metal Library (#{t.name} from #{t.source})"
  system("xcrun metallib -o #{t.name.shellescape} #{t.source.shellescape}") or exit
end

rule ".rs" => -> (f) { [XML_RS_MAP.fetch(f), "#{__dir__}/generator/objc-bridge-generator.rb"] } do |t|
  puts "Generating ObjC bridge (#{t.name} from #{t.source})"

  io = StringIO.new
  Bridge.generate(File.read(t.source), Bridge::Output.new(io))
  File.write(t.name, io.string)
end
