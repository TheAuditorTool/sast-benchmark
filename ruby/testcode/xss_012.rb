require_relative 'shared'

def sanitize_strict(input)
  input.to_s.gsub(/<[^>]*>/, '')
            .gsub(/javascript:/i, '')
            .gsub(/on\w+\s*=/i, '')
end

# vuln-code-snippet start ruby_xss_sanitize_strict
def xss_sanitize_strict(req)
  bio = req.post('bio')
  display_name = req.post('display_name')
  clean_bio = sanitize_strict(bio) # vuln-code-snippet safe-line ruby_xss_sanitize_strict
  clean_name = sanitize_strict(display_name)
  BenchmarkResponse.html("<section><h2>#{clean_name}</h2><p>#{clean_bio}</p></section>")
end
# vuln-code-snippet end ruby_xss_sanitize_strict
