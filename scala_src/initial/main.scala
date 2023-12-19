package initial

enum Exploration:
  case Explored, UnExplored

enum Maze:
  case Branch(label: String, left: Maze, right: Maze, var status: Exploration = Exploration.UnExplored)
  case Leaf(label: String)

  def explore: List[String] = this match
    case branch@Branch(label, left, right, status) =>
      status match
        case Exploration.UnExplored =>
          branch.status = Exploration.Explored
          label :: left.explore ::: right.explore
        case Exploration.Explored => List(label)
    case Leaf(label) => List(label)

@main
def main(): Unit = {
  // Construction of the example
  val leaf2 = Maze.Leaf("2")
  val leaf4 = Maze.Leaf("4")
  val leaf5 = Maze.Leaf("5")
  val leaf8 = Maze.Leaf("8")
  val branch3 = Maze.Branch("3", leaf4, leaf5)
  val branch1 = Maze.Branch("1", leaf2, branch3)
  val branch7 = Maze.Branch("7", leaf5, leaf8)
  val branch6 = Maze.Branch("6", branch3, branch7)
  val branch0 = Maze.Branch("0", branch1, branch6)

  // Exploration of the maze
  val trace = branch0.explore
  println(trace)
}


