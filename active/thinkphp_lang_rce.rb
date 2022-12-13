# encoding: utf-8
require 'uri'
require "unicode_normalize/normalize.rb"
class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "thinkphp_lang_rce", 
            "description" => "" }
        return grades
    end

    def scan(url)
        client = HttpClient.new
        resp = client.get(url, headers={"host"=>"bing.com"})
        puts resp
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

    def passive_run(index)
        puts $LOADED_FEATURES.select { |feature| feature.include? 'gems' }.map { |feature| File.dirname(feature) }.map { |feature| feature.split('/').last }.uniq.sort
        log = HistoryLog.get_req index
        url = log['url']
        debug("Test u好rl: #{url}")
        uri = URI::parse(url)
        debug("Test url: #{uri}")

        #scan("http://127.0.0.1:8009")
    end

end

