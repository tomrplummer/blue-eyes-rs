require "toml-rb"
require_relative '../plugins/route_builder'
require_relative '../plugins/paths'

module PathsHelper
  extend RouteBuilder
  include Paths
  def self.run
    config = TomlRB.load_file(File.expand_path('./helpers/paths_config.toml'))

    as_lookup = {}

    unless config["resources"].nil?
      config["resources"].each do |resource|
        as_lookup[resource["name"]] = resource["as"] || resource["name"]
      end
      puts "lookup #{as_lookup}"
      config["resources"].each do |resource|
        #if resource["as"].nil?
        #resources resource["name"].to_sym
          #else
        resources resource["name"].to_sym, :as => (resource["as"] ? resource["as"].to_sym : nil), :belongs_to => (resource["belongs_to"] ? as_lookup[resource["belongs_to"]] : nil)
          #end
      end
    end

    # put resources here
  end
end
