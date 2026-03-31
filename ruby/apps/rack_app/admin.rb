require_relative '../../testcode/shared'
require 'json'
require 'securerandom'

# rack_app - Admin panel

# vuln-code-snippet start rk_deser_marshal
def restore_user_session(env)
  req = Rack::Request.new(env)
  cookie_data = req.cookies['user_session']
  decoded = [cookie_data].pack('H*')
  obj = Marshal.load(decoded) # vuln-code-snippet vuln-line rk_deser_marshal
  [200, { 'Content-Type' => 'text/plain' }, [obj.to_s]]
end
# vuln-code-snippet end rk_deser_marshal

# vuln-code-snippet start rk_deser_json
def restore_user_session_safe(env)
  req = Rack::Request.new(env)
  cookie_data = req.cookies['user_session']
  obj = JSON.parse(cookie_data) # vuln-code-snippet safe-line rk_deser_json
  [200, { 'Content-Type' => 'application/json' }, [JSON.generate(obj)]]
end
# vuln-code-snippet end rk_deser_json

# vuln-code-snippet start rk_fileupload_no_validate
def upload_avatar(env)
  req = Rack::Request.new(env)
  upload = req.params['avatar']
  original_name = upload[:filename]
  File.write("/var/blog/avatars/#{original_name}", upload[:tempfile].read) # vuln-code-snippet vuln-line rk_fileupload_no_validate
  [200, { 'Content-Type' => 'text/plain' }, ['uploaded']]
end
# vuln-code-snippet end rk_fileupload_no_validate

# vuln-code-snippet start rk_fileupload_validated
def upload_avatar_safe(env)
  req = Rack::Request.new(env)
  upload = req.params['avatar']
  original_name = upload[:filename].to_s
  ext = File.extname(original_name).downcase
  return [400, { 'Content-Type' => 'text/plain' }, ['invalid file type']] unless %w[.jpg .jpeg .png .gif].include?(ext) # vuln-code-snippet safe-line rk_fileupload_validated
  safe_name = "#{SecureRandom.uuid}#{ext}"
  File.write("/var/blog/avatars/#{safe_name}", upload[:tempfile].read)
  [200, { 'Content-Type' => 'text/plain' }, [safe_name]]
end
# vuln-code-snippet end rk_fileupload_validated
