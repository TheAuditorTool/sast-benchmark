require_relative 'shared'

class SalaryRecord
  attr_reader :id, :employee_id, :amount

  def initialize(id, employee_id, amount)
    @id = id
    @employee_id = employee_id
    @amount = amount
  end

  def self.find(id)
    new(id, "emp_#{id.to_i % 5}", id.to_i * 1_000)
  end
end

def policy_authorized?(user_id, record_employee_id)
  user_id == record_employee_id || user_id == 'hr_admin'
end

def resolve_salary(obj, args, ctx)
  user_id = ctx[:current_user_id]
  record = SalaryRecord.find(args[:id])
  raise 'Forbidden' unless policy_authorized?(user_id, record.employee_id)
  record
end
