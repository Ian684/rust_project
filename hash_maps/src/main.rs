#[derive(Clone,Debug)]
enum Exercise07State{Idle,Running,Finished}

struct Exercise07Record{id:usize,label:String,score:i32,state:Exercise07State}

impl Exercise07Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_07_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+7)).sum()
}

fn main(){
    println!("Exercise 07: hash_maps");
    let records=[
        Exercise07Record{id:1,label:"Ada".into(),score:98,state:Exercise07State::Finished},
        Exercise07Record{id:2,label:"Linus".into(),score:86,state:Exercise07State::Running},
        Exercise07Record{id:3,label:"Grace".into(),score:73,state:Exercise07State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_07_operation(&[12,7,19,3,25]));
}

const EXERCISE_07_27:i32=7;
const EXERCISE_07_29:i32=7;
const EXERCISE_07_30:i32=7;
const EXERCISE_07_31:i32=7;
const EXERCISE_07_32:i32=7;
const EXERCISE_07_33:i32=7;
const EXERCISE_07_34:i32=7;
const EXERCISE_07_35:i32=7;
const EXERCISE_07_36:i32=7;
const EXERCISE_07_37:i32=7;
const EXERCISE_07_38:i32=7;
const EXERCISE_07_39:i32=7;
const EXERCISE_07_40:i32=7;
const EXERCISE_07_41:i32=7;
const EXERCISE_07_42:i32=7;
const EXERCISE_07_43:i32=7;
const EXERCISE_07_44:i32=7;
const EXERCISE_07_45:i32=7;
const EXERCISE_07_46:i32=7;
const EXERCISE_07_47:i32=7;
const EXERCISE_07_48:i32=7;
const EXERCISE_07_49:i32=7;
const EXERCISE_07_50:i32=7;
const EXERCISE_07_51:i32=7;
const EXERCISE_07_52:i32=7;
const EXERCISE_07_53:i32=7;
const EXERCISE_07_54:i32=7;
const EXERCISE_07_55:i32=7;
const EXERCISE_07_56:i32=7;
const EXERCISE_07_57:i32=7;
const EXERCISE_07_58:i32=7;
const EXERCISE_07_59:i32=7;
const EXERCISE_07_60:i32=7;
const EXERCISE_07_61:i32=7;
const EXERCISE_07_62:i32=7;
const EXERCISE_07_63:i32=7;
const EXERCISE_07_64:i32=7;
const EXERCISE_07_65:i32=7;
const EXERCISE_07_66:i32=7;
const EXERCISE_07_67:i32=7;
const EXERCISE_07_68:i32=7;
const EXERCISE_07_69:i32=7;
const EXERCISE_07_70:i32=7;
const EXERCISE_07_71:i32=7;
const EXERCISE_07_72:i32=7;
const EXERCISE_07_73:i32=7;
const EXERCISE_07_74:i32=7;
const EXERCISE_07_75:i32=7;
const EXERCISE_07_76:i32=7;
const EXERCISE_07_77:i32=7;
const EXERCISE_07_78:i32=7;
const EXERCISE_07_79:i32=7;
