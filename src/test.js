let b = { 
  val : 3,
  getVal: function() {
    return this.val
  }
}

let x = b.getVal()

let smth = b.getVal

var val = 77

let x1 = b.getVal

console.log(x1())