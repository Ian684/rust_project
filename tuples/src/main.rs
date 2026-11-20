#[derive(Clone,Debug)]
enum Exercise26State{Idle,Running,Finished}

struct Exercise26Record{id:usize,label:String,score:i32,state:Exercise26State}

impl Exercise26Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_26_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+26)).sum()
}

fn main(){
    println!("Exercise 26: tuples");
    let records=[
        Exercise26Record{id:1,label:"Ada".into(),score:98,state:Exercise26State::Finished},
        Exercise26Record{id:2,label:"Linus".into(),score:86,state:Exercise26State::Running},
        Exercise26Record{id:3,label:"Grace".into(),score:73,state:Exercise26State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_26_operation(&[12,7,19,3,25]));
}

const EXERCISE_26_27:i32=26;
const EXERCISE_26_29:i32=26;
const EXERCISE_26_30:i32=26;
const EXERCISE_26_31:i32=26;
const EXERCISE_26_32:i32=26;
const EXERCISE_26_33:i32=26;
const EXERCISE_26_34:i32=26;
const EXERCISE_26_35:i32=26;
const EXERCISE_26_36:i32=26;
const EXERCISE_26_37:i32=26;
const EXERCISE_26_38:i32=26;
const EXERCISE_26_39:i32=26;
const EXERCISE_26_40:i32=26;
const EXERCISE_26_41:i32=26;
const EXERCISE_26_42:i32=26;
const EXERCISE_26_43:i32=26;
const EXERCISE_26_44:i32=26;
const EXERCISE_26_45:i32=26;
const EXERCISE_26_46:i32=26;
const EXERCISE_26_47:i32=26;
const EXERCISE_26_48:i32=26;
const EXERCISE_26_49:i32=26;
const EXERCISE_26_50:i32=26;
const EXERCISE_26_51:i32=26;
const EXERCISE_26_52:i32=26;
const EXERCISE_26_53:i32=26;
const EXERCISE_26_54:i32=26;
const EXERCISE_26_55:i32=26;
const EXERCISE_26_56:i32=26;
const EXERCISE_26_57:i32=26;
const EXERCISE_26_58:i32=26;
const EXERCISE_26_59:i32=26;
const EXERCISE_26_60:i32=26;
const EXERCISE_26_61:i32=26;
const EXERCISE_26_62:i32=26;
const EXERCISE_26_63:i32=26;
const EXERCISE_26_64:i32=26;
const EXERCISE_26_65:i32=26;
const EXERCISE_26_66:i32=26;
const EXERCISE_26_67:i32=26;
const EXERCISE_26_68:i32=26;
const EXERCISE_26_69:i32=26;
const EXERCISE_26_70:i32=26;
const EXERCISE_26_71:i32=26;
const EXERCISE_26_72:i32=26;
const EXERCISE_26_73:i32=26;
const EXERCISE_26_74:i32=26;
const EXERCISE_26_75:i32=26;
const EXERCISE_26_76:i32=26;
const EXERCISE_26_77:i32=26;
const EXERCISE_26_78:i32=26;
const EXERCISE_26_79:i32=26;
