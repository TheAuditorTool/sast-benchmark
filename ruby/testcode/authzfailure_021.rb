require_relative 'shared'

class Comment
  def self.find(id)
    { id: id, body: "comment body #{id}", user_id: "user_#{id.to_i % 5}" }
  end
end

# vuln-code-snippet start ruby_authz_idor_comment
def get_comment(req)
  comment_id = req.param('id')
  comment = Comment.find(comment_id) # vuln-code-snippet vuln-line ruby_authz_idor_comment
  BenchmarkResponse.json(comment)
end
# vuln-code-snippet end ruby_authz_idor_comment
