require_relative 'shared'

begin
  require 'zip'
rescue LoadError
  # rubyzip gem not available -- tests are syntax-checked, not executed
  module Zip
    class File
      def self.open(path)
        yield new
      end
      def each; end
    end
  end
end

# vuln-code-snippet start ruby_pt_zip_extract
def zip_extract(req)
  Zip::File.open(req.param('archive')) { |z| z.each { |e| e.extract } } # vuln-code-snippet vuln-line ruby_pt_zip_extract
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_pt_zip_extract
