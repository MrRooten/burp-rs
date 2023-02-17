class UriParser 
    @@parser = RBUriParser.new
    def self.parse(s) 
        Uri.new(@@parser.parse(s))
    end
end

class Uri 
    def initialize hash 
        @@scheme = hash['scheme']
        @@host = hash['host']
        @@path = hash['path']
        @@query = hash['query']
        @@port = hash['port']
    end

    def scheme
        @@scheme
    end

    def host_with_port
        "#{@@host}:#{@@port}"
    end

    def domain_with_scheme
        "#{@@scheme}://#{@@host}:#{@@port}"
    end

    def host
        @@host
    end

    def path 
        @@path
    end

    def query
        @@query
    end

    def port 
        @@port 
    end
    def path_increment_with_slash
        path = @@path.split('/')
        increment = "/"
        path.each do |item|
            increment += item + "/"
            yield increment
        end
    end

    def path_increment 
        path = @@path.split("/")
        increment = ""
        path.each do |item|
            increment += "/"
            increment += item
            yield increment
        end
    end
end