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

def serve_rack_file(env)
  Rack::File.new('/app/public').call(env)
end
