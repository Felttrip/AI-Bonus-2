package aima.gui.demo;

import aima.core.agent.Action;
import aima.core.agent.Agent;
import aima.core.agent.Environment;
import aima.core.agent.EnvironmentView;
import aima.core.agent.impl.SimpleEnvironmentView;
import aima.core.environment.vacuum.ModelBasedReflexVacuumAgent;
import aima.core.environment.vacuum.VacuumEnvironment;
import aima.core.environment.wumpusworld.WumpusCave;
import aima.core.environment.wumpusworld.*;
import aima.core.logic.propositional.parsing.ast.Sentence;

/**
 * Created by ecgprc on 11/8/14.
 */
public class WumpusDemo {

    public static void main(String[] args) {

        /* Init The Enviornment with the map in the book
        *  Note that we created this environment
        *
        * */
        WumpusEnv env = new WumpusEnv();
        HybridWumpusAgent agent = env.getAgent();

        String init_map = "";


        /* Start in 1,1 with the percepts all set to false as in the book */
        Action pos =  agent.execute(new AgentPercept(false, false, false, false, false));


        
        /* Start Useful code for future use */

 //      AgentPosition cur = agent.kb.askCurrentPosition(agent.t);


//       for( Sentence str : agent.kb.getSentences()){
//           System.out.println(str);
//       }

//       agent.execute(new AgentPercept(false,true,false,false,false));
//       agent.execute(new AgentPercept(true,false,false,false,false));
//       agent.execute(new AgentPercept(true,true,true,false,false));
    }
}


