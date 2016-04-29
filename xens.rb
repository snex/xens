require 'sinatra'
require 'sinatra/flash'
require 'sinatra/redirect_with_flash'
require 'sinatra/activerecord'
require_relative './environments'

enable :sessions
disable :protection

class ShortURL < ActiveRecord::Base

  validates_uniqueness_of :url_hash
  validates_uniqueness_of :url

  def self.generate_hash url
    chars = ('a'..'z').to_a + ('A'..'Z').to_a + ('0'..'9').to_a + ['-', '_']
    chars.sample(5).join
  end

end

get '/' do
  haml :index
end

post '/create' do
  @short_url = ShortURL.find_or_initialize_by(url: params['url'])
  if @short_url.new_record?
    @short_url.url_hash = ShortURL.generate_hash(params['url'])
  end
  if @short_url.save
    redirect '/', :notice => "Short URL: <a href='http://xens.org/#{@short_url.url_hash}'>http://xens.org/#{@short_url.url_hash}</a>"
  else
    redirect '/', :error => 'Failed to generate Short URL. Try again.'
  end
end

get /^\/([A-Za-z0-9\-_]{5}$)/ do |q|
  short_url = ShortURL.find_by_url_hash q
  if short_url
    redirect short_url.url
  else
    redirect '/'
  end
end
