package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"regexp"

	"github.com/BurntSushi/toml"
)

type GitFileObject struct {
	Name        string `json:"name"`
	DownloadUrl string `json:"download_url"`
	Type        string `json:"type"`
	Links       struct {
		Self string `json:"self"`
	} `json:"_links"`
}

type TomlSPFile struct {
	Dependencies []struct {
		URL  string `toml:"url"`
		Path string `toml:"path"`
	} `toml:"dependency"`
}

var token string

func main() {
	token = getGithubToken()

	var tomlFile TomlSPFile
	_, err := toml.DecodeFile("sp.toml", &tomlFile)

	if err != nil {
		log.Fatalln(err.Error())

		return
	}

	for _, dependency := range tomlFile.Dependencies {
		APIURL := getAPIURL(dependency.URL)

		fmt.Println(dependency.URL)

		download(APIURL, "scripting/include/"+dependency.Path)
	}
}

func download(url string, directory string) {
	jsonContent, err := getContentsFromURL(url)

	if err != nil {
		log.Fatalln("Error: " + err.Error())

		return
	}

	err = os.MkdirAll(directory, 0755)

	if err != nil {
		log.Fatalln("There has been an error in creating the directory for the include: " + err.Error())

		return
	}

	var result []GitFileObject
	err = json.Unmarshal([]byte(jsonContent), &result)

	// It could be that there is only one file in the url ( so it's not an array of objects but just an object ).
	// We handle it here.
	if err != nil {
		var temp GitFileObject

		err = json.Unmarshal([]byte(jsonContent), &temp)

		result = []GitFileObject{temp}

		if err != nil {
			log.Fatalln("There has been an error in getting the json object. Error: " + err.Error())
		}
	}

	for _, f := range result {
		if f.Type == "dir" {
			if err != nil {
				log.Fatalln("There has been an error in creating the directory for the include: " + err.Error())

				return
			}

			download(f.Links.Self, directory+"/"+f.Name)

			continue
		}

		if filepath.Ext(f.Name) != ".inc" {
			continue
		}

		websiteContents, err := getContentsFromURL(f.DownloadUrl)

		if err != nil {
			log.Fatalln(err.Error())

			return
		}

		err = os.WriteFile(directory+"/"+f.Name, websiteContents, 0666)

		fmt.Println("Downloading: " + directory + "/" + f.Name)

		if err != nil {
			log.Fatalln("There has been an error while trying to create the file for: " + f.Name + ". Error: " + err.Error())

			return
		}
	}
}

func getContentsFromURL(url string) ([]byte, error) {
	client := &http.Client{}

	req, err := http.NewRequest("GET", url, nil)

	if err != nil {
		fmt.Println("Error creating request: ", err.Error())

		return nil, err
	}

	if token != "" {
		req.Header.Set("Authorization", "Token "+token)
	}

	// Send the HTTP request
	resp, err := client.Do(req)

	if err != nil {
		fmt.Println("Error sending request: ", err.Error())

		return nil, err
	}

	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)

	return body, nil
}

func getAPIURL(url string) string {
	regexNameOfFile, err := regexp.Compile("https://github.com/([^/]+)/([^/]+)(?:/(?:tree|blob)/([^/]+)/?)?(.*)?$")

	if err != nil {
		log.Fatalln("regex error")

		return ""
	}

	match := regexNameOfFile.FindStringSubmatch(url)

	var repository string

	if match[3] != "" {
		repository = "?ref=" + match[3]
	}

	return "https://api.github.com/repos/" + match[1] + "/" + match[2] + "/contents" + match[4] + repository
}

func getGithubToken() string {
	args := os.Args

	if len(args) > 1 {
		return args[1]
	} else {
		return ""
	}
}
