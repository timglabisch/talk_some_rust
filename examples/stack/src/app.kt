
interface Greetable {
    fun greet();
}

class Person : Greetable {
    override fun greet() {
        println("greet");
    }
}

fun main(args: Array<String>) {
    val person : Greetable = Person();
    person.greet();
}
