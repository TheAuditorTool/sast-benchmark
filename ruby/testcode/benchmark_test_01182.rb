require_relative 'shared'

class CurrentUserModel
  def initialize(id)
    @id = id
    @comments = CommentCollection.new(id)
  end

  def comments
    @comments
  end
end

class CommentCollection
  def initialize(owner_id)
    @owner_id = owner_id
  end

  def find(id)
    comment_owner = "user_#{id.to_i % 5}"
    raise 'not found' unless comment_owner == @owner_id
    { id: id, body: "comment #{id}", user_id: @owner_id }
  end
end

def get_my_comment(req)
  comment_id = req.param('id')
  current_user = CurrentUserModel.new(req.cookie('user_id'))
  comment = current_user.comments.find(comment_id)
  BenchmarkResponse.json(comment)
end
