# encoding: utf-8
require 'uri'
require "unicode_normalize/normalize.rb"
class RBModule_cati_front_rce
    def initialize
    end
    def metadata
        grades = { "name" => "cacti_front_rce", 
            "description" => "" }
        return grades
    end

    def scan(uri)
        scheme = uri["scheme"]
        if scheme == nil 
            error("Scheme is none #{uri}")
            return 
        end

        url = scheme + "://" + uri["host"] + uri["path"] + "?lang="
        client = Request.new
        resp = client.get(url, headers={"host"=>uri["host"]})
        #puts resp
        if false 
            issue = {
                "name"=> "test_req",
                "level" => "info",
                "confidence" => "confirm",
                "detail" => "this is a detail",
                "host" => url,
                "response" => resp
            }
        
            issuer = RBIssue.new
            issuer.push_issue(issue)
        end
        
    end

    def passive_run(index)
        log = HistoryLog.get_req index
        url = log['url']
        uri = UriParser.parse(url)
        info("Test url: #{uri}")
        scan(uri)
        #scan("http://127.0.0.1:8009")
    end

end

