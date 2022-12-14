class UriParser 
    @@parser = RBUriParser.new
    def self.parse(s) 
        @@parser.parse(s)
    end
end