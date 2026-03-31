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

# vuln-code-snippet start ruby_xss_to_s_html
def xss_to_s_html(req)
  display_name = req.param('display_name')
  bio = req.param('bio')
  profile = UserProfile.new(display_name, bio)
  card = "<div class=\"profile\"><h2>#{profile.to_s}</h2><p>#{profile.bio}</p></div>" # vuln-code-snippet vuln-line ruby_xss_to_s_html
  BenchmarkResponse.html(card)
end
# vuln-code-snippet end ruby_xss_to_s_html
