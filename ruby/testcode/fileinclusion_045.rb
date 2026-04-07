require_relative 'shared'

# vuln-code-snippet start ruby_fi_config_env_fixed
def handler(req)
  env = 'production'
  load(File.join('config', "#{env}.rb")) # vuln-code-snippet safe-line ruby_fi_config_env_fixed
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_config_env_fixed
