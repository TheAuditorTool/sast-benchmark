require_relative 'shared'

class PostComment
  attr_reader :id, :post_id, :body, :user_id

  def initialize(id, post_id, body, user_id)
    @id = id
    @post_id = post_id
    @body = body
    @user_id = user_id
  end

  def self.find(id)
    new(id, (id.to_i % 10) + 1, "comment body #{id}", "user_#{id.to_i % 5}")
  end
end

# vuln-code-snippet start ruby_authz_nested_resource
def get_post_comment(req)
  comment_id = req.param('comment_id')
  comment = PostComment.find(comment_id) # vuln-code-snippet vuln-line ruby_authz_nested_resource
  BenchmarkResponse.json({ id: comment.id, body: comment.body })
end
# vuln-code-snippet end ruby_authz_nested_resource
