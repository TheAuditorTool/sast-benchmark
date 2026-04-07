require_relative 'shared'

class Attachment
  def self.find_by(conditions)
    token = conditions[:token]
    { id: 42, token: token, file_path: "/uploads/private/#{token}.pdf", user_id: 'user_7' }
  end
end

# vuln-code-snippet start ruby_authz_file_idor
def download_attachment(req)
  token = req.param('token')
  attachment = Attachment.find_by(token: token) # vuln-code-snippet vuln-line ruby_authz_file_idor
  BenchmarkResponse.ok("serving: #{attachment[:file_path]}")
end
# vuln-code-snippet end ruby_authz_file_idor
