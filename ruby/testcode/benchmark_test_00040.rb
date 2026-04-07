require_relative 'shared'

class Attachment
  def self.find_by(conditions)
    token = conditions[:token]
    { id: 42, token: token, file_path: "/uploads/private/#{token}.pdf", user_id: 'user_7' }
  end
end

def download_attachment(req)
  token = req.param('token')
  attachment = Attachment.find_by(token: token)
  BenchmarkResponse.ok("serving: #{attachment[:file_path]}")
end
