require_relative 'shared'

class Widget
  attr_reader :id, :owner_id

  def initialize(id, owner_id)
    @id = id
    @owner_id = owner_id
  end
end

def authorize_resource!(user_id, resource)
  raise 'Access denied' unless resource.owner_id == user_id
end

# vuln-code-snippet start ruby_authz_cancan_load_resource
def show_widget(req)
  widget_id = req.param('id')
  current_user_id = req.cookie('user_id')
  widget = Widget.new(widget_id, "user_#{widget_id.to_i % 5}")
  authorize_resource!(current_user_id, widget) # vuln-code-snippet safe-line ruby_authz_cancan_load_resource
  BenchmarkResponse.json({ id: widget.id })
end
# vuln-code-snippet end ruby_authz_cancan_load_resource
