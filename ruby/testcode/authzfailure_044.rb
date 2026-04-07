require_relative 'shared'

@_pundit_policy_authorized = false

def pundit_authorize(user_id, record_owner_id)
  @_pundit_policy_authorized = (user_id == record_owner_id)
end

def verify_authorized!
  raise 'Authorization not performed' unless @_pundit_policy_authorized
end

class Ticket
  def self.find(id)
    { id: id, subject: "ticket #{id}", owner_id: "user_#{id.to_i % 5}" }
  end
end

# vuln-code-snippet start ruby_authz_pundit_authorize_all
def show_ticket(req)
  ticket_id = req.param('id')
  current_user_id = req.cookie('user_id')
  ticket = Ticket.find(ticket_id)
  pundit_authorize(current_user_id, ticket[:owner_id])
  verify_authorized! # vuln-code-snippet safe-line ruby_authz_pundit_authorize_all
  BenchmarkResponse.json(ticket)
end
# vuln-code-snippet end ruby_authz_pundit_authorize_all
