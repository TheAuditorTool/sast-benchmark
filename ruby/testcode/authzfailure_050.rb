require_relative 'shared'

class Forbidden < StandardError; end

class Ability
  def initialize(user_id)
    @user_id = user_id
  end

  def can?(action, resource)
    return true if action == :read && resource[:user_id] == @user_id
    false
  end
end

class PrivateNote
  def self.find(id)
    { id: id, content: "note #{id}", user_id: "user_#{id.to_i % 5}" }
  end
end

# vuln-code-snippet start ruby_authz_ability_check
def read_note(req)
  note_id = req.param('id')
  current_user_id = req.cookie('user_id')
  ability = Ability.new(current_user_id)
  resource = PrivateNote.find(note_id)
  ability.can?(:read, resource) || raise(Forbidden) # vuln-code-snippet safe-line ruby_authz_ability_check
  BenchmarkResponse.json(resource)
end
# vuln-code-snippet end ruby_authz_ability_check
