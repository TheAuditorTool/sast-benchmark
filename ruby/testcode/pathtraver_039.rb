require_relative 'shared'

module Rack
  class File
    def initialize(root)
      @root = root
    end
    def call(env)
      [200, { 'Content-Type' => 'application/octet-stream' }, []]
    end
  end
end

# vuln-code-snippet start ruby_pt_rack_file_fixed_root
def serve_rack_file(env)
  Rack::File.new('/app/public').call(env) # vuln-code-snippet safe-line ruby_pt_rack_file_fixed_root
end
# vuln-code-snippet end ruby_pt_rack_file_fixed_root
