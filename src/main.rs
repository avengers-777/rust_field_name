use field_names::FieldNames;
/*
这是一个多行注释的例子。
可以包含多行文本。
适用于更长的注释，解释复杂的代码或算法。
1. 9913 6296		 6. 2886 2097
2. 4689 0477		 7. 6299 9985
3. 3035 9854		 8. 2625 4321
4. 5793 1500		 9. 6517 7275
5. 4198 8839		10. 3975 2214

*/

#[derive(FieldNames)]
struct Book {
    author: String,
    title: String,
    year_published: u32,
}
impl Book {
    pub fn print_details(&self) {
        println!("Author: {}, Title: {}, Year: {}", self.author, self.title, self.year_published);
    }
}
#[derive(FieldNames)]
struct Person {
    name: String,
    address: String,
    age: u32,
}
fn main() {
    let book = Book {
        author: String::from("J.R.R. Tolkien"),
        title: String::from("The Lord of the Rings"),
        year_published: 1954,
    };
    println!("Field name for 'author': {}", Book::author);
    println!("Field name for 'title': {}", Book::title);
    println!("Field name for 'year_published': {}", Book::year_published);
    println!("Field name for 'age': {}", Person::age);
    println!("Field name for 'name': {}", Person::name);
    println!("Field name for 'address': {}", Person::address);
    book.print_details();
}