#[derive(Clone,Debug)]
enum Exercise03State{Idle,Running,Finished}

struct Exercise03Record{id:usize,label:String,score:i32,state:Exercise03State}

impl Exercise03Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_03_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+3)).sum()
}

fn main(){
    println!("Exercise 03: binary_tree");
    let records=[
        Exercise03Record{id:1,label:"Ada".into(),score:98,state:Exercise03State::Finished},
        Exercise03Record{id:2,label:"Linus".into(),score:86,state:Exercise03State::Running},
        Exercise03Record{id:3,label:"Grace".into(),score:73,state:Exercise03State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_03_operation(&[12,7,19,3,25]));
}

const EXERCISE_03_27:i32=3;
const EXERCISE_03_29:i32=3;
const EXERCISE_03_30:i32=3;
const EXERCISE_03_31:i32=3;
const EXERCISE_03_32:i32=3;
const EXERCISE_03_33:i32=3;
const EXERCISE_03_34:i32=3;
const EXERCISE_03_35:i32=3;
const EXERCISE_03_36:i32=3;
const EXERCISE_03_37:i32=3;
const EXERCISE_03_38:i32=3;
const EXERCISE_03_39:i32=3;
const EXERCISE_03_40:i32=3;
const EXERCISE_03_41:i32=3;
const EXERCISE_03_42:i32=3;
const EXERCISE_03_43:i32=3;
const EXERCISE_03_44:i32=3;
const EXERCISE_03_45:i32=3;
const EXERCISE_03_46:i32=3;
const EXERCISE_03_47:i32=3;
const EXERCISE_03_48:i32=3;
const EXERCISE_03_49:i32=3;
const EXERCISE_03_50:i32=3;
const EXERCISE_03_51:i32=3;
const EXERCISE_03_52:i32=3;
const EXERCISE_03_53:i32=3;
const EXERCISE_03_54:i32=3;
const EXERCISE_03_55:i32=3;
const EXERCISE_03_56:i32=3;
const EXERCISE_03_57:i32=3;
const EXERCISE_03_58:i32=3;
const EXERCISE_03_59:i32=3;
const EXERCISE_03_60:i32=3;
const EXERCISE_03_61:i32=3;
const EXERCISE_03_62:i32=3;
const EXERCISE_03_63:i32=3;
const EXERCISE_03_64:i32=3;
const EXERCISE_03_65:i32=3;
const EXERCISE_03_66:i32=3;
const EXERCISE_03_67:i32=3;
const EXERCISE_03_68:i32=3;
const EXERCISE_03_69:i32=3;
const EXERCISE_03_70:i32=3;
const EXERCISE_03_71:i32=3;
const EXERCISE_03_72:i32=3;
const EXERCISE_03_73:i32=3;
const EXERCISE_03_74:i32=3;
const EXERCISE_03_75:i32=3;
const EXERCISE_03_76:i32=3;
const EXERCISE_03_77:i32=3;
const EXERCISE_03_78:i32=3;
const EXERCISE_03_79:i32=3;
