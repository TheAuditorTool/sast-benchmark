require_relative 'shared'

module Rails
  module Application
    def self.config
      OpenStruct.new(session_store: :cookie_store)
    end
  end
end

# vuln-code-snippet start ruby_securecookie_rails_session
def configure_rails_session(req)
  config = { key: '_app_session', secure: true, httponly: true, same_site: :strict } # vuln-code-snippet safe-line ruby_securecookie_rails_session
  BenchmarkResponse.ok("configured: #{config}")
end
# vuln-code-snippet end ruby_securecookie_rails_session
