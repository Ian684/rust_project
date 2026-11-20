#[derive(Clone,Debug)]
enum Exercise13State{Idle,Running,Finished}

struct Exercise13Record{id:usize,label:String,score:i32,state:Exercise13State}

impl Exercise13Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_13_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+13)).sum()
}

fn main(){
    println!("Exercise 13: modules");
    let records=[
        Exercise13Record{id:1,label:"Ada".into(),score:98,state:Exercise13State::Finished},
        Exercise13Record{id:2,label:"Linus".into(),score:86,state:Exercise13State::Running},
        Exercise13Record{id:3,label:"Grace".into(),score:73,state:Exercise13State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_13_operation(&[12,7,19,3,25]));
}

const EXERCISE_13_27:i32=13;
const EXERCISE_13_29:i32=13;
const EXERCISE_13_30:i32=13;
const EXERCISE_13_31:i32=13;
const EXERCISE_13_32:i32=13;
const EXERCISE_13_33:i32=13;
const EXERCISE_13_34:i32=13;
const EXERCISE_13_35:i32=13;
const EXERCISE_13_36:i32=13;
const EXERCISE_13_37:i32=13;
const EXERCISE_13_38:i32=13;
const EXERCISE_13_39:i32=13;
const EXERCISE_13_40:i32=13;
const EXERCISE_13_41:i32=13;
const EXERCISE_13_42:i32=13;
const EXERCISE_13_43:i32=13;
const EXERCISE_13_44:i32=13;
const EXERCISE_13_45:i32=13;
const EXERCISE_13_46:i32=13;
const EXERCISE_13_47:i32=13;
const EXERCISE_13_48:i32=13;
const EXERCISE_13_49:i32=13;
const EXERCISE_13_50:i32=13;
const EXERCISE_13_51:i32=13;
const EXERCISE_13_52:i32=13;
const EXERCISE_13_53:i32=13;
const EXERCISE_13_54:i32=13;
const EXERCISE_13_55:i32=13;
const EXERCISE_13_56:i32=13;
const EXERCISE_13_57:i32=13;
const EXERCISE_13_58:i32=13;
const EXERCISE_13_59:i32=13;
const EXERCISE_13_60:i32=13;
const EXERCISE_13_61:i32=13;
const EXERCISE_13_62:i32=13;
const EXERCISE_13_63:i32=13;
const EXERCISE_13_64:i32=13;
const EXERCISE_13_65:i32=13;
const EXERCISE_13_66:i32=13;
const EXERCISE_13_67:i32=13;
const EXERCISE_13_68:i32=13;
const EXERCISE_13_69:i32=13;
const EXERCISE_13_70:i32=13;
const EXERCISE_13_71:i32=13;
const EXERCISE_13_72:i32=13;
const EXERCISE_13_73:i32=13;
const EXERCISE_13_74:i32=13;
const EXERCISE_13_75:i32=13;
const EXERCISE_13_76:i32=13;
const EXERCISE_13_77:i32=13;
const EXERCISE_13_78:i32=13;
const EXERCISE_13_79:i32=13;
