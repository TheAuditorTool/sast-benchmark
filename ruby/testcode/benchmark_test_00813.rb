require_relative 'shared'

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

def zip_extract(req)
  Zip::File.open(req.param('archive')) { |z| z.each { |e| e.extract } }
  BenchmarkResponse.json({ ok: true })
end
