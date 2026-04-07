require_relative 'shared'

class UserProfile
  attr_accessor :display_name, :bio

  def initialize(display_name, bio)
    @display_name = display_name
    @bio = bio
  end

  def to_s
    @display_name
  end
end

def handler(req)
  display_name = req.param('display_name')
  bio = req.param('bio')
  profile = UserProfile.new(display_name, bio)
  card = "<div class=\"profile\"><h2>#{profile.to_s}</h2><p>#{profile.bio}</p></div>"
  BenchmarkResponse.html(card)
end
