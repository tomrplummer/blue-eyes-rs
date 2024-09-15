module PermittedParams
  def self.apply(model)
    model.instance_eval do
      def self.permitted(params)
        columns = self.columns.filter { |c| c != :id }
        allowed_params = {}

        columns.each do |column|
          if params[column]
            allowed_params[column] = params[column]
          end
        end

        allowed_params
      end
    end
  end
end
