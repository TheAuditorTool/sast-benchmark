require_relative 'shared'
require 'logger'

# vuln-code-snippet start ruby_loginj_progname_constant
def log_progname_constant(req)
  Logger.new($stdout, progname: 'MyApp').info("request processed") # vuln-code-snippet safe-line ruby_loginj_progname_constant
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_progname_constant
