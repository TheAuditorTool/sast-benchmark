require_relative 'shared'
require 'securerandom'

def render_form(req)
  token = SecureRandom.hex(32)
  form_html = "<input type='hidden' name='csrf' value='#{token}'>"
  BenchmarkResponse.html("<form>#{form_html}<input type='submit'></form>")
end
