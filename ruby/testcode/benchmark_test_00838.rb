require_relative 'shared'
require 'pathname'

begin
  require 'zip'
rescue LoadError
  module Zip
    class File
      def self.open(path)
        yield new
      end
      def each; end
    end
  end
end

def zip_safe_extract(req)
  base = '/var/extracts'
  Zip::File.open('/uploads/safe.zip') do |z|
    z.each do |e|
      dest = File.join(base, e.name)
      raise unless Pathname.new(dest).cleanpath.to_s.start_with?(base)
      e.extract(dest)
    end
  end
  BenchmarkResponse.json({ ok: true })
end
