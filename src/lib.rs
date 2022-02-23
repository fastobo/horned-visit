#![recursion_limit = "1024"]

#[macro_use]
extern crate paste;
#[macro_use]
extern crate blanket;
extern crate horned_owl;

use std::collections::BTreeSet;
use horned_owl::model::*;

macro_rules! impl_traits {
    ($visit:ident, $($name:ident($type:ty),)*) => {
        paste! {
            #[blanket(default = "visit", derive(Mut, Box))]
            pub trait $visit<'ast> {
                /// Visit the annotations of an [`AnnotatedAxiom`].
                fn visit_annotations(&mut self, annotations: &'ast BTreeSet<Annotation>);
                $(
                    #[doc = "Visit a [`" $type "`]."]
                    fn [<visit_ $name>] (&mut self, $name: &'ast $type);
                )*
            }

            #[blanket(default = "visit_mut", derive(Mut, Box))]
            pub trait [<$visit Mut>] {
                /// Visit the annotations of an [`AnnotatedAxiom`].
                fn visit_annotations(&mut self, annotations: &mut BTreeSet<Annotation>);
                $(
                    #[doc = "Visit a [`" $type "`]."]
                    fn [<visit_ $name>] (&mut self, $name: &mut $type);
                )*
            }
        }
    }
}

impl_traits! { Visit,
    annotated_axiom(AnnotatedAxiom),
    annotation(Annotation),
    annotation_assertion(AnnotationAssertion),
    annotation_property(AnnotationProperty),
    annotation_property_domain(AnnotationPropertyDomain),
    annotation_property_range(AnnotationPropertyRange),
    annotation_subject(AnnotationSubject),
    annotation_value(AnnotationValue),
    anonymous_individual(AnonymousIndividual),
    asymmetric_object_property(AsymmetricObjectProperty),
    axiom(Axiom),
    class(Class),
    class_assertion(ClassAssertion),
    class_expression(ClassExpression),
    data_property(DataProperty),
    data_property_assertion(DataPropertyAssertion),
    data_property_domain(DataPropertyDomain),
    data_property_range(DataPropertyRange),
    data_range(DataRange),
    datatype(Datatype),
    datatype_definition(DatatypeDefinition),
    declare_annotation_property(DeclareAnnotationProperty),
    declare_class(DeclareClass),
    declare_datatype(DeclareDatatype),
    declare_data_property(DeclareDataProperty),
    declare_named_individual(DeclareNamedIndividual),
    declare_object_property(DeclareObjectProperty),
    different_individuals(DifferentIndividuals),
    disjoint_classes(DisjointClasses),
    disjoint_data_properties(DisjointDataProperties),
    disjoint_object_properties(DisjointObjectProperties),
    disjoint_union(DisjointUnion),
    equivalent_classes(EquivalentClasses),
    equivalent_data_properties(EquivalentDataProperties),
    equivalent_object_properties(EquivalentObjectProperties),
    facet(Facet),
    facet_restriction(FacetRestriction),
    functional_data_property(FunctionalDataProperty),
    functional_object_property(FunctionalObjectProperty),
    has_key(HasKey),
    import(Import),
    individual(Individual),
    inverse_functional_object_property(InverseFunctionalObjectProperty),
    inverse_object_properties(InverseObjectProperties),
    iri(IRI),
    irreflexive_object_property(IrreflexiveObjectProperty),
    literal(Literal),
    named_individual(NamedIndividual),
    negative_data_property_assertion(NegativeDataPropertyAssertion),
    negative_object_property_assertion(NegativeObjectPropertyAssertion),
    object_property(ObjectProperty),
    object_property_assertion(ObjectPropertyAssertion),
    object_property_expression(ObjectPropertyExpression),
    object_property_domain(ObjectPropertyDomain),
    object_property_range(ObjectPropertyRange),
    ontology_annotation(OntologyAnnotation),
    property_expression(PropertyExpression),
    reflexive_object_property(ReflexiveObjectProperty),
    same_individual(SameIndividual),
    sub_class_of(SubClassOf),
    sub_annotation_property_of(SubAnnotationPropertyOf),
    sub_data_property_of(SubDataPropertyOf),
    sub_object_property_expression(SubObjectPropertyExpression),
    sub_object_property_of(SubObjectPropertyOf),
    symmetric_object_property(SymmetricObjectProperty),
    transitive_object_property(TransitiveObjectProperty),
}

macro_rules! impl_default {
    ( $visitor:ident, $($name:ident($type:ty) => $code:expr,)* ) => {
        paste! {
            /// Default implementations of the `Visit` methods.
            pub mod visit {

                use super::*;

                macro_rules! r { ($x:expr) => (&$x); }

                #[allow(unused_variables)]
                pub fn visit_annotations<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, annotations: &'ast BTreeSet<Annotation>) {
                    annotations.iter().for_each(|a| visitor.visit_annotation(a));
                }

                $(#[allow(unused_variables)]
                pub fn [<visit_ $name>] <'ast, V: Visit<'ast> + ?Sized>($visitor: &mut V, $name: &'ast $type) {
                    $code
                })*
            }

            /// Default implementations of the `VisitMut` methods.
            pub mod visit_mut {

                use super::*;

                macro_rules! r { ($x:expr) => (&mut $x); }

                #[allow(unused_variables)]
                pub fn visit_annotations<V: VisitMut + ?Sized>($visitor: &mut V, annotations: &mut BTreeSet<Annotation>) {
                    // cannot mutably visit the elements of a BTreeSet
                }

                $(#[allow(unused_variables)]
                pub fn [<visit_ $name>] <V: VisitMut + ?Sized>($visitor: &mut V, $name: &mut $type) {
                    $code
                })*
            }
        }
    }
}

impl_default! { visitor,

    annotated_axiom(AnnotatedAxiom) => {
        visitor.visit_annotations(r!(annotated_axiom.ann));
        visitor.visit_axiom(r!(annotated_axiom.axiom));
    },

    annotation(Annotation) => {
        visitor.visit_annotation_property(r!(annotation.ap));
        visitor.visit_annotation_value(r!(annotation.av));
    },

    annotation_assertion(AnnotationAssertion) => {
        visitor.visit_annotation_subject(r!(annotation_assertion.subject));
        visitor.visit_annotation(r!(annotation_assertion.ann));
    },

    annotation_property(AnnotationProperty) => {
        visitor.visit_iri(r!(annotation_property.0));
    },

    annotation_property_domain(AnnotationPropertyDomain) => {
        visitor.visit_annotation_property(r!(annotation_property_domain.ap));
        visitor.visit_iri(r!(annotation_property_domain.iri));
    },

    annotation_property_range(AnnotationPropertyRange) => {
        visitor.visit_annotation_property(r!(annotation_property_range.ap));
        visitor.visit_iri(r!(annotation_property_range.iri));
    },

    annotation_subject(AnnotationSubject) => {
        use self::AnnotationSubject::*;
        match annotation_subject {
            IRI(iri) => visitor.visit_iri(r!(*iri)),
            AnonymousIndividual(ai) => visitor.visit_anonymous_individual(r!(*ai)),
        }
    },

    annotation_value(AnnotationValue) => {
        match annotation_value {
            AnnotationValue::Literal(lit) => visitor.visit_literal(r!(*lit)),
            AnnotationValue::IRI(iri) => visitor.visit_iri(r!(*iri)),
        }
    },

    anonymous_individual(AnonymousIndividual) => {},

    asymmetric_object_property(AsymmetricObjectProperty) => {
        visitor.visit_object_property_expression(r!(asymmetric_object_property.0));
    },

    axiom(Axiom) => {
        use self::Axiom::*;
        match axiom {
            OntologyAnnotation(oa) => visitor.visit_ontology_annotation(r!(*oa)),
            Import(import) => visitor.visit_import(r!(*import)),
            DeclareClass(dc) => visitor.visit_declare_class(r!(*dc)),
            DeclareObjectProperty(dop) => visitor.visit_declare_object_property(r!(*dop)),
            DeclareAnnotationProperty(dap) => visitor.visit_declare_annotation_property(r!(*dap)),
            DeclareDataProperty(ddp) => visitor.visit_declare_data_property(r!(*ddp)),
            DeclareNamedIndividual(dni) => visitor.visit_declare_named_individual(r!(*dni)),
            DeclareDatatype(ddt) => visitor.visit_declare_datatype(r!(*ddt)),
            SubClassOf(sco) => visitor.visit_sub_class_of(r!(*sco)),
            EquivalentClasses(ec) => visitor.visit_equivalent_classes(r!(*ec)),
            DisjointClasses(dc) => visitor.visit_disjoint_classes(r!(*dc)),
            DisjointUnion(du) => visitor.visit_disjoint_union(r!(*du)),
            SubObjectPropertyOf(sopo) => visitor.visit_sub_object_property_of(r!(*sopo)),
            EquivalentObjectProperties(eop) => visitor.visit_equivalent_object_properties(r!(*eop)),
            DisjointObjectProperties(dop) => visitor.visit_disjoint_object_properties(r!(*dop)),
            InverseObjectProperties(iop) => visitor.visit_inverse_object_properties(r!(*iop)),
            ObjectPropertyDomain(opd) => visitor.visit_object_property_domain(r!(*opd)),
            ObjectPropertyRange(opr) => visitor.visit_object_property_range(r!(*opr)),
            FunctionalObjectProperty(fop) => visitor.visit_functional_object_property(r!(*fop)),
            InverseFunctionalObjectProperty(ifop) => visitor.visit_inverse_functional_object_property(r!(*ifop)),
            ReflexiveObjectProperty(rop) => visitor.visit_reflexive_object_property(r!(*rop)),
            IrreflexiveObjectProperty(iop) => visitor.visit_irreflexive_object_property(r!(*iop)),
            SymmetricObjectProperty(sop) => visitor.visit_symmetric_object_property(r!(*sop)),
            AsymmetricObjectProperty(aop) => visitor.visit_asymmetric_object_property(r!(*aop)),
            TransitiveObjectProperty(top) => visitor.visit_transitive_object_property(r!(*top)),
            SubDataPropertyOf(sdpo) => visitor.visit_sub_data_property_of(r!(*sdpo)),
            EquivalentDataProperties(edp) => visitor.visit_equivalent_data_properties(r!(*edp)),
            DisjointDataProperties(ddp) => visitor.visit_disjoint_data_properties(r!(*ddp)),
            DataPropertyDomain(dpd) => visitor.visit_data_property_domain(r!(*dpd)),
            DataPropertyRange(dpr) => visitor.visit_data_property_range(r!(*dpr)),
            FunctionalDataProperty(fdp) => visitor.visit_functional_data_property(r!(*fdp)),
            DatatypeDefinition(dd) => visitor.visit_datatype_definition(r!(*dd)),
            HasKey(hk) => visitor.visit_has_key(r!(*hk)),
            SameIndividual(si) => visitor.visit_same_individual(r!(*si)),
            DifferentIndividuals(di) => visitor.visit_different_individuals(r!(*di)),
            ClassAssertion(ca) => visitor.visit_class_assertion(r!(*ca)),
            ObjectPropertyAssertion(opa) => visitor.visit_object_property_assertion(r!(*opa)),
            NegativeObjectPropertyAssertion(nopa) => visitor.visit_negative_object_property_assertion(r!(*nopa)),
            DataPropertyAssertion(dpa) => visitor.visit_data_property_assertion(r!(*dpa)),
            NegativeDataPropertyAssertion(ndpa) => visitor.visit_negative_data_property_assertion(r!(*ndpa)),
            AnnotationAssertion(aa) => visitor.visit_annotation_assertion(r!(*aa)),
            SubAnnotationPropertyOf(sapo) => visitor.visit_sub_annotation_property_of(r!(*sapo)),
            AnnotationPropertyDomain(apd) => visitor.visit_annotation_property_domain(r!(*apd)),
            AnnotationPropertyRange(apr) => visitor.visit_annotation_property_range(r!(*apr)),
        }
    },

    class(Class) => {
        visitor.visit_iri(r!(class.0));
    },

    class_assertion(ClassAssertion) => {
        visitor.visit_class_expression(r!(class_assertion.ce));
        visitor.visit_individual(r!(class_assertion.i));
    },

    class_expression(ClassExpression) => {
        use self::ClassExpression::*;
        match class_expression {
            Class(c) => visitor.visit_class(r!(*c)),
            ObjectIntersectionOf(ces) |
            ObjectUnionOf(ces) => {
                for ce in r!(*ces).into_iter() {
                    visitor.visit_class_expression(ce);
                }
            }
            ObjectComplementOf(ce) => visitor.visit_class_expression(r!(*ce)),
            ObjectOneOf(is) => {
                for i in r!(*is).into_iter() {
                    visitor.visit_individual(i);
                }
            }
            ObjectHasSelf(ope) => visitor.visit_object_property_expression(r!(*ope)),
            ObjectSomeValuesFrom { ope, bce }      |
            ObjectAllValuesFrom { ope, bce }       |
            ObjectMinCardinality { ope, bce, .. }   |
            ObjectMaxCardinality { ope, bce, .. }   |
            ObjectExactCardinality { ope, bce, .. } => {
                visitor.visit_object_property_expression(r!(*ope));
                visitor.visit_class_expression(r!(*bce));
            },
            ObjectHasValue { ope, i } => {
                visitor.visit_object_property_expression(r!(*ope));
                visitor.visit_individual(r!(*i));
            },
            DataSomeValuesFrom { dp, dr }       |
            DataAllValuesFrom { dp, dr }       |
            DataMinCardinality { dp, dr, .. }   |
            DataMaxCardinality { dp, dr, .. }   |
            DataExactCardinality { dp, dr, .. } => {
                visitor.visit_data_property(r!(*dp));
                visitor.visit_data_range(r!(*dr));
            },
            DataHasValue { dp, l } => {
                visitor.visit_data_property(r!(*dp));
                visitor.visit_literal(r!(*l));
            },
        }
    },

    data_property(DataProperty) => {
        visitor.visit_iri(r!(data_property.0));
    },

    data_property_assertion(DataPropertyAssertion) => {
        visitor.visit_data_property(r!(data_property_assertion.dp));
        visitor.visit_individual(r!(data_property_assertion.from));
        visitor.visit_literal(r!(data_property_assertion.to));
    },

    data_property_domain(DataPropertyDomain) => {
        visitor.visit_data_property(r!(data_property_domain.dp));
        visitor.visit_class_expression(r!(data_property_domain.ce));
    },

    data_property_range(DataPropertyRange) => {
        visitor.visit_data_property(r!(data_property_range.dp));
        visitor.visit_data_range(r!(data_property_range.dr));
    },

    data_range(DataRange) => {
        use self::DataRange::*;
        match data_range {
            Datatype(dt) => visitor.visit_datatype(r!(*dt)),
            DataComplementOf(dr) => visitor.visit_data_range(r!(*dr)),
            DataIntersectionOf(drs) => {
                for dr in r!(*drs).into_iter() {
                    visitor.visit_data_range(dr);
                }
            }
            DataUnionOf(drs) => {
                for dr in r!(*drs).into_iter() {
                    visitor.visit_data_range(dr);
                }
            }
            DataOneOf(lits) => {
                for lit in r!(*lits).into_iter() {
                    visitor.visit_literal(lit);
                }
            }
            DatatypeRestriction(dt, frs) => {
                visitor.visit_datatype(dt);
                for fr in r!(*frs).into_iter() {
                    visitor.visit_facet_restriction(fr);
                }
            }
        }
    },

    datatype(Datatype) => {
        visitor.visit_iri(r!(datatype.0));
    },

    datatype_definition(DatatypeDefinition) => {
        visitor.visit_datatype(r!(datatype_definition.kind));
        visitor.visit_data_range(r!(datatype_definition.range));
    },

    declare_annotation_property(DeclareAnnotationProperty) => {
        visitor.visit_annotation_property(r!(declare_annotation_property.0));
    },

    declare_class(DeclareClass) => {
        visitor.visit_class(r!(declare_class.0));
    },

    declare_datatype(DeclareDatatype) => {
        visitor.visit_datatype(r!(declare_datatype.0));
    },

    declare_data_property(DeclareDataProperty) => {
        visitor.visit_data_property(r!(declare_data_property.0));
    },

    declare_named_individual(DeclareNamedIndividual) => {
        visitor.visit_named_individual(r!(declare_named_individual.0));
    },

    declare_object_property(DeclareObjectProperty) => {
        visitor.visit_object_property(r!(declare_object_property.0));
    },

    different_individuals(DifferentIndividuals) => {
        for i in r!(different_individuals.0).into_iter() {
            visitor.visit_individual(i);
        }
    },

    disjoint_classes(DisjointClasses) => {
        for ce in r!(disjoint_classes.0).into_iter() {
            visitor.visit_class_expression(ce);
        }
    },

    disjoint_data_properties(DisjointDataProperties) => {
        for dp in r!(disjoint_data_properties.0).into_iter() {
            visitor.visit_data_property(dp);
        }
    },

    disjoint_object_properties(DisjointObjectProperties) => {
        for ope in r!(disjoint_object_properties.0).into_iter() {
            visitor.visit_object_property_expression(ope);
        }
    },

    disjoint_union(DisjointUnion) => {
        visitor.visit_class(r!(disjoint_union.0));
        for ce in r!(disjoint_union.1).into_iter() {
            visitor.visit_class_expression(ce);
        }
    },

    equivalent_classes(EquivalentClasses) => {
        for ce in r!(equivalent_classes.0).into_iter() {
            visitor.visit_class_expression(ce);
        }
    },

    equivalent_data_properties(EquivalentDataProperties) => {
        for dp in r!(equivalent_data_properties.0).into_iter() {
            visitor.visit_data_property(dp);
        }
    },

    equivalent_object_properties(EquivalentObjectProperties) => {
        for ope in r!(equivalent_object_properties.0).into_iter() {
            visitor.visit_object_property_expression(ope);
        }
    },

    facet(Facet) => {},

    facet_restriction(FacetRestriction) => {
        visitor.visit_facet(r!(facet_restriction.f));
        visitor.visit_literal(r!(facet_restriction.l));
    },

    functional_data_property(FunctionalDataProperty) => {
        visitor.visit_data_property(r!(functional_data_property.0));
    },

    functional_object_property(FunctionalObjectProperty) => {
        visitor.visit_object_property_expression(r!(functional_object_property.0));
    },

    has_key(HasKey) => {
        visitor.visit_class_expression(r!(has_key.ce));
        for pe in r!(has_key.vpe).into_iter() {
            visitor.visit_property_expression(pe);
        }
    },

    import(Import) => {
        visitor.visit_iri(r!(import.0));
    },

    individual(Individual) => {
        use self::Individual::*;
        match individual {
            Anonymous(a) => visitor.visit_anonymous_individual(r!(*a)),
            Named(n) => visitor.visit_named_individual(r!(*n)),
        }
    },

    inverse_functional_object_property(InverseFunctionalObjectProperty) => {
        visitor.visit_object_property_expression(r!(inverse_functional_object_property.0));
    },

    inverse_object_properties(InverseObjectProperties) => {
        visitor.visit_object_property(r!(inverse_object_properties.0));
        visitor.visit_object_property(r!(inverse_object_properties.1));
    },

    irreflexive_object_property(IrreflexiveObjectProperty) => {
        visitor.visit_object_property_expression(r!(irreflexive_object_property.0));
    },

    iri(IRI) => {},

    literal(Literal) => {},

    named_individual(NamedIndividual) => {
        visitor.visit_iri(r!(named_individual.0));
    },

    negative_data_property_assertion(NegativeDataPropertyAssertion) => {
        visitor.visit_data_property(r!(negative_data_property_assertion.dp));
        visitor.visit_individual(r!(negative_data_property_assertion.from));
        visitor.visit_literal(r!(negative_data_property_assertion.to));
    },

    negative_object_property_assertion(NegativeObjectPropertyAssertion) => {
        visitor.visit_object_property_expression(r!(negative_object_property_assertion.ope));
        visitor.visit_individual(r!(negative_object_property_assertion.from));
        visitor.visit_individual(r!(negative_object_property_assertion.to));
    },

    object_property(ObjectProperty) => {
        visitor.visit_iri(r!(object_property.0));
    },

    object_property_assertion(ObjectPropertyAssertion) => {
        visitor.visit_object_property_expression(r!(object_property_assertion.ope));
        visitor.visit_individual(r!(object_property_assertion.from));
        visitor.visit_individual(r!(object_property_assertion.to));
    },

    object_property_domain(ObjectPropertyDomain) => {
        visitor.visit_object_property_expression(r!(object_property_domain.ope));
        visitor.visit_class_expression(r!(object_property_domain.ce));
    },

    object_property_expression(ObjectPropertyExpression) => {
        use self::ObjectPropertyExpression::*;
        match object_property_expression {
            ObjectProperty(op) => visitor.visit_object_property(r!(*op)),
            InverseObjectProperty(op) => visitor.visit_object_property(r!(*op)),
        }
    },

    object_property_range(ObjectPropertyRange) => {
        visitor.visit_object_property_expression(r!(object_property_range.ope));
        visitor.visit_class_expression(r!(object_property_range.ce));
    },

    ontology_annotation(OntologyAnnotation) => {
        visitor.visit_annotation(r!(ontology_annotation.0));
    },

    property_expression(PropertyExpression) => {
        use self::PropertyExpression::*;
        match property_expression {
            ObjectPropertyExpression(ope) => visitor.visit_object_property_expression(r!(*ope)),
            DataProperty(dp) => visitor.visit_data_property(r!(*dp)),
            AnnotationProperty(ap) => visitor.visit_annotation_property(r!(*ap)),
        }
    },

    reflexive_object_property(ReflexiveObjectProperty) => {
        visitor.visit_object_property_expression(r!(reflexive_object_property.0));
    },

    same_individual(SameIndividual) => {
        for i in r!(same_individual.0).into_iter() {
            visitor.visit_individual(i);
        }
    },

    sub_annotation_property_of(SubAnnotationPropertyOf) => {
        visitor.visit_annotation_property(r!(sub_annotation_property_of.sup));
        visitor.visit_annotation_property(r!(sub_annotation_property_of.sub));
    },

    sub_class_of(SubClassOf) => {
        visitor.visit_class_expression(r!(sub_class_of.sup));
        visitor.visit_class_expression(r!(sub_class_of.sub));
    },

    sub_data_property_of(SubDataPropertyOf) => {
        visitor.visit_data_property(r!(sub_data_property_of.sup));
        visitor.visit_data_property(r!(sub_data_property_of.sub));
    },

    sub_object_property_expression(SubObjectPropertyExpression) => {
        use self::SubObjectPropertyExpression::*;
        match sub_object_property_expression {
            ObjectPropertyExpression(ope) => visitor.visit_object_property_expression(r!(*ope)),
            ObjectPropertyChain(opes) => {
                for ope in r!(*opes).into_iter() {
                    visitor.visit_object_property_expression(ope);
                }
            }
        }
    },

    sub_object_property_of(SubObjectPropertyOf) => {
        visitor.visit_object_property_expression(r!(sub_object_property_of.sup));
        visitor.visit_sub_object_property_expression(r!(sub_object_property_of.sub));
    },

    symmetric_object_property(SymmetricObjectProperty) => {
        visitor.visit_object_property_expression(r!(symmetric_object_property.0));
    },

    transitive_object_property(TransitiveObjectProperty) => {
        visitor.visit_object_property_expression(r!(transitive_object_property.0));
    },
}
