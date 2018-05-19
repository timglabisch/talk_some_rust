class Person(private val age : Int) {
    fun getAge() {
        return this.age;
    }
}

fun main(args: Array<String>) {
    val tim = Person(25);
    System.out.println("tim is " + tim.getAge() + " years old")
}