require_relative 'shared'

class Invoice
  attr_reader :id, :user_id, :amount

  def initialize(id, user_id, amount)
    @id = id
    @user_id = user_id
    @amount = amount
  end

  def self.find(id)
    new(id, "user_#{id.to_i % 5}", id.to_i * 100)
  end
end

def authorize!(user_id, record)
  raise 'Forbidden' unless record.user_id == user_id
end

def show_invoice(req)
  invoice_id = req.param('id')
  current_user_id = req.cookie('user_id')
  invoice = Invoice.find(invoice_id)
  authorize!(current_user_id, invoice)
  BenchmarkResponse.json({ id: invoice.id, amount: invoice.amount })
end
