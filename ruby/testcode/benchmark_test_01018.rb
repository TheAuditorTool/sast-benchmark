require_relative 'shared'

class Comment
  def self.find(id)
    { id: id, body: "comment body #{id}", user_id: "user_#{id.to_i % 5}" }
  end
end

def get_comment(req)
  comment_id = req.param('id')
  comment = Comment.find(comment_id)
  BenchmarkResponse.json(comment)
end
