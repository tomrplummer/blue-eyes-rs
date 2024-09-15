require 'active_support/core_ext/string/inflections'
require_relative './paths'

module RouteBuilder
  extend ActiveSupport::Inflector
  def resources(resource, options = nil)
    resource = resource.to_s
    resource_as = options && options[:as] ? options[:as].to_s : resource

    Paths.define_method :"get_#{resource}_path" do |args = nil|
      path = ''
      path += "/#{options[:belongs_to]}/#{args[:id]}" if options && options[:belongs_to]
      path + "/#{resource_as}"
    end

    Paths.define_method :"get_#{resource}_route" do
      path = ''
      path += "/#{options[:belongs_to]}/:#{options[:belongs_to].to_s.singularize}_id" if options && options[:belongs_to]
      path + "/#{resource_as}"
    end

    Paths.define_method :"create_#{resource.singularize}_path" do |args = nil|
      path = ''
      path += "/#{options[:belongs_to]}/#{args[:id]}" if options && options[:belongs_to]
      path + "/#{resource_as}"
    end

    Paths.define_method :"create_#{resource.singularize}_route" do
      path = ''
      path += "/#{options[:belongs_to]}/:#{options[:belongs_to].to_s.singularize}_id" if options && options[:belongs_to]
      path + "/#{resource_as}"
    end

    Paths.define_method :"get_#{resource.singularize}_path" do |args|
      "/#{resource_as}/#{args[:id]}"
    end

    Paths.define_method :"get_#{resource.singularize}_route" do
      "/#{resource_as}/:id"
    end

    Paths.define_method :"edit_#{resource.singularize}_path" do |args|
      "/#{resource_as}/#{args[:id]}/edit"
    end

    Paths.define_method :"edit_#{resource.singularize}_route" do
      "/#{resource_as}/:id/edit"
    end

    Paths.define_method :"update_#{resource.singularize}_path" do |args|
      "/#{resource_as}/#{args[:id]}"
    end

    Paths.define_method :"update_#{resource.singularize}_route" do
      "/#{resource_as}/:id"
    end

    Paths.define_method :"new_#{resource.singularize}_path" do |args = nil|
      path = ''
      path += "/#{options[:belongs_to]}/#{args[:id]}" if options && options[:belongs_to]
      path + "/#{resource_as}/new"
    end

    Paths.define_method :"new_#{resource.singularize}_route" do
      path = ''
      path += "/#{options[:belongs_to]}/:#{options[:belongs_to].to_s.singularize}_id" if options && options[:belongs_to]
      path + "/#{resource_as}/new"
    end
  end
end
