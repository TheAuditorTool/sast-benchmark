require_relative 'shared'

module Rails
  module Application
    def self.config
      OpenStruct.new(session_store: :cookie_store)
    end
  end
end

def configure_rails_session(req)
  config = { key: '_app_session', secure: true, httponly: true, same_site: :strict }
  BenchmarkResponse.ok("configured: #{config}")
end
