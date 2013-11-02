class CreateShorturls < ActiveRecord::Migration
  def up
    create_table :short_urls do |t|
      t.string :url_hash
      t.string :url
      t.timestamps
    end

    add_index :short_urls, :url_hash, :unique => true
    add_index :short_urls, :url,      :unique => true
  end

  def down
    drop_table :short_urls
  end
end
