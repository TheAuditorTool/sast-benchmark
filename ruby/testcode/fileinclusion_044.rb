require_relative 'shared'

# vuln-code-snippet start ruby_fi_gem_require_only
def handler(req)
  Gem.find_files('plugins/*.rb').each { |f| require f } # vuln-code-snippet safe-line ruby_fi_gem_require_only
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_gem_require_only
