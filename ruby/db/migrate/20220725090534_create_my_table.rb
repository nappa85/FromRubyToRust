class CreateMyTable < ActiveRecord::Migration[6.1]
  def change
    create_table :my_table do |t|

      t.string 'status'
      t.string 'metadata'
    end
  end
end
