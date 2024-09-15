require 'haml'

class HomeController < ApplicationController
  get '/' do
    respond_to do
      json {{message: "Coming Soon"}}
      html {haml :home_index}
    end
  end
end
