class PopulateMyTable < ActiveRecord::Migration[6.1]
  def change
    MyModel.create(status: 'Active', metadata: {})
    parent = MyModel.first
    MyModel.create(status: 'Active', metadata: {
      parent_id: parent.id
    })
  end
end
