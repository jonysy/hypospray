use {Co, Component, ComponentImp, Construct, Graph, Prototype, };

pub trait Resolve<'imp, T, K> where T: ?Sized {
    
    type CoImp;
    
    fn __resolve(&'imp self) -> Self::CoImp;
}

impl<'imp, M, T> Resolve<'imp, T, Prototype> for Graph<M>
    where M: ?Sized + Component<T>, 
          T: ?Sized,
          Graph<M>: Dependencies<'imp, Co<M, T>> {
    
    type CoImp = Co<M, T>;
    
    fn __resolve(&'imp self) -> Self::CoImp {
        
        Dependencies::<'imp, Co<M, T>>::__dependencies(self)
    }
}

pub trait Dependencies<'imp, D> {
    
    fn __dependencies(&'imp self) -> D;
}

impl<'imp, M> Dependencies<'imp, ()> for Graph<M> where M: ?Sized {
    
    fn __dependencies(&'imp self) { }
}

impl<'imp, M, T> Dependencies<'imp, Co<M, T>> for Graph<M>
    where M: ?Sized + Component<T, Scope=Prototype>,
          M::ComponentImp: Construct<'imp>,
          T: ?Sized,
          Graph<M>: Dependencies<'imp, <M::ComponentImp as Construct<'imp>>::Dep> {
    
    fn __dependencies(&'imp self) -> Co<M, T> {
        
        Co(
            <M::ComponentImp as Construct>::__construct(
                self.__dependencies()
            )
        )
    }
}