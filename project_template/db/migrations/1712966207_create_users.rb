Sequel.migration do
  change do
    create_table(:users) do
      primary_key :id
      String :full_name
      String :username
      String :password_hash
      String :role
    end
  end
end
