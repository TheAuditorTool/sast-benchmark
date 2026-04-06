require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_csrf_per_form_token
def render_form(req)
  token = SecureRandom.hex(32)
  form_html = "<input type='hidden' name='csrf' value='#{token}'>" # vuln-code-snippet safe-line ruby_csrf_per_form_token
  BenchmarkResponse.html("<form>#{form_html}<input type='submit'></form>")
end
# vuln-code-snippet end ruby_csrf_per_form_token
