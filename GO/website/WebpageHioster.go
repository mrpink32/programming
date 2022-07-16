package main

import (
	"fmt"
	"html/template"
	"log"
	"net/http"
	"os"
	"regexp"
)

type Page struct {
	Title string
	Body  []byte
}

var templates = template.Must(template.ParseFiles("edit.html", "Index5.html", "Index.html", "homepage.html"))
var validPath = regexp.MustCompile("^/(edit|save|view)/([a-zA-Z0-9]+)$")

// func (p *Page) save() error {
// 	filename := p.Title + ".txt"
// 	return os.WriteFile(filename, p.Body, 0600)
// }

func loadPage(title string) (*Page, error) {
	filename := title + ".html"
	body, err := os.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	return &Page{Title: title, Body: body}, nil
}

func homeHandler(writer http.ResponseWriter, request *http.Request) {
	page, err := loadPage("Index5")
	if err != nil {
		fmt.Println(err)
	}
	//tmpl, _ := template.ParseFiles(page.Title)
	//tmpl.Execute(writer, nil)
	templates.ExecuteTemplate(writer, page.Title+".html", page)
}

func renderTemplate(writer http.ResponseWriter, tmpl string, page *Page) {
	templates.ExecuteTemplate(writer, tmpl+".html", page)
}

// func viewHandler(writer http.ResponseWriter, request *http.Request, title string) {
// 	page, err := loadPage(title)
// 	if err != nil {
// 		http.Redirect(writer, request, "/edit/"+title, http.StatusFound)
// 		return
// 	}
// 	renderTemplate(writer, "view", page)
// }

// func editHandler(writer http.ResponseWriter, request *http.Request, title string) {
// 	page, err := loadPage(title)
// 	if err != nil {
// 		page = &Page{Title: title}
// 	}
// 	renderTemplate(writer, "edit", page)
// }

// func saveHandler(writer http.ResponseWriter, request *http.Request, title string) {
// 	body := request.FormValue("body")
// 	page := &Page{Title: title, Body: []byte(body)}
// 	err := page.save()
// 	if err != nil {
// 		http.Error(writer, err.Error(), http.StatusInternalServerError)
// 		return
// 	}
// 	http.Redirect(writer, request, "/view/"+title, http.StatusFound)
// }

func makeHandler(fn func(http.ResponseWriter, *http.Request, string)) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// Here we will extract the page title from the Request,
		// and call the provided handler 'fn'
		m := validPath.FindStringSubmatch(r.URL.Path)
		if m == nil {
			http.NotFound(w, r)
			return
		}
		fn(w, r, m[2]) // The title is the second subexpression.
	}
}

func main() {
	http.HandleFunc("/", homeHandler)
	// http.HandleFunc("/view/", makeHandler(viewHandler))
	// http.HandleFunc("/edit/", makeHandler(editHandler))
	// http.HandleFunc("/save/", makeHandler(saveHandler))
	log.Fatal(http.ListenAndServe(":80", nil))
}
